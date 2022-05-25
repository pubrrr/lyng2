import {
    decode,
    EnumVariant,
    string,
    tupleStruct,
    TupleStruct,
} from "./parser";
import {
    apply,
    buildLexer,
    err,
    expectEOF,
    expectSingleResult,
    seq,
    str,
    tok,
    Token,
} from "typescript-parsec";

type Enum = EnumVariant<"Success", string> | EnumVariant<"Error", number>;

enum TokenKind {
    Identifier,
    String,
    LeftParenthesis,
    RightParenthesis,
    DoubleQuote,
}

function decode2(input: string): TupleStruct<"Success", [string]> {
    const lexer = buildLexer([
        [true, /^[a-zA-Z][a-zA-Z\d]*/g, TokenKind.Identifier],
        [true, /^[^")(]*/g, TokenKind.String],
        [true, /^\(/g, TokenKind.LeftParenthesis],
        [true, /^\)/g, TokenKind.RightParenthesis],
        [true, /^"/g, TokenKind.DoubleQuote],
    ]);

    let tokens = lexer.parse(input);
    let output = seq(
        str("Success"),
        err(tok(TokenKind.LeftParenthesis), "expected ("),
        tok(TokenKind.DoubleQuote),
        apply(
            tok(TokenKind.String),
            (value: Token<TokenKind>): string => value.text
        ),
        tok(TokenKind.DoubleQuote),
        tok(TokenKind.RightParenthesis)
    ).parse(tokens);
    let result = expectSingleResult(expectEOF(output));

    return {
        name: "Success",
        value: [result[3]],
    };
}

test("tuple with one field", () => {
    let result = decode2('Success("success message")');

    expect(result.value).toEqual(["success message"]);
});

describe("decoding RON strings", () => {
    test("should succeed on a valid string", () => {
        let result = decode('"valid string"', string);

        expect(result.success).toBe(true);
        expect((result as { value: string }).value).toBe("valid string");
    });

    test("should fail with missing quotes", () => {
        let result = decode("has no quotes", string);

        expect(result.success).toBe(false);
        expect((result as { error: string }).error).toBe(
            "expected opening double quotes"
        );
    });

    test("should fail with missing closing double quotes", () => {
        let result = decode('"has no closing quotes', string);

        expect(result.success).toBe(false);
        expect((result as { error: string }).error).toBe(
            "expected closing double quotes"
        );
    });

    test("should fail on double quotes in the middle", () => {
        let result = decode('"misplaced " quote"', string);

        expect(result.success).toBe(false);
    });
});

describe("decoding RON tuple structs", () => {
    test("should decode a valid tuple struct with one string field", () => {
        let underTest = tupleStruct("Success", [string]);

        let result = decode('Success("success message")', underTest);

        expect(result.success).toBe(true);
        expect(
            (result as { value: TupleStruct<"Success", string[]> }).value
        ).toEqual({
            name: "Success",
            value: ["success message"],
        });
    });
});
