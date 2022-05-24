import { EnumVariant, TupleStruct } from "./parser";
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

function decode(input: string): TupleStruct<"Success", [string]> {
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
    let result = decode('Success("success message")');

    expect(result.value).toEqual(["success message"]);
});
