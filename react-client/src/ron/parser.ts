import {
    apply,
    buildLexer,
    expectEOF,
    kleft,
    kright,
    opt,
    Parser,
    ParserOutput,
    seq,
    str,
    tok,
    Token,
} from "typescript-parsec";

enum RonToken {
    Identifier,
    String,
    LeftParenthesis,
    RightParenthesis,
    Whitespace,
    Comma,
}

const RON_LEXER = buildLexer([
    [true, /^[a-zA-Z][a-zA-Z\d]*/g, RonToken.Identifier],
    [true, /^"[^")(]*"/g, RonToken.String],
    [true, /^\s+/g, RonToken.Whitespace],
    [true, /^,/g, RonToken.Comma],
    [true, /^\(/g, RonToken.LeftParenthesis],
    [true, /^\)/g, RonToken.RightParenthesis],
]);

interface RonDecoder<V> {
    (token: Token<RonToken> | undefined): ParserOutput<RonToken, V>;
}

export type Result<V> =
    | {
          success: true;
          value: V;
      }
    | {
          success: false;
          error: string;
      };

export function decode<V>(input: string, decoder: RonDecoder<V>): Result<V> {
    let token;
    try {
        token = RON_LEXER.parse(input);
    } catch (e) {
        return {
            success: false,
            error: e instanceof Error ? e.message : "could not lex RON input",
        };
    }
    const parseResult = expectEOF(decoder(token));

    if (!parseResult.successful) {
        return { success: false, error: parseResult.error.message };
    }
    if (parseResult.candidates.length == 0) {
        return {
            success: false,
            error: "Decoding RON value returned no result",
        };
    }
    if (parseResult.candidates.length > 1) {
        return {
            success: false,
            error: "Decoding RON value returned ambiguous values",
        };
    }
    return { success: true, value: parseResult.candidates[0].result };
}

export const string = apply(
    tok(RonToken.String),
    (token: Token<RonToken>): string => token.text.slice(1, -1)
).parse;

const comma = seq(
    opt(tok(RonToken.Whitespace)),
    str<RonToken>(","),
    opt(tok(RonToken.Whitespace))
);

export function tupleStruct<Literal extends string, Value extends any[]>(
    name: StringLiteral<Literal>,
    fields: FieldsArray<Value>
): RonDecoder<TupleStruct<Literal, Value>> {
    return (
        token: Token<RonToken> | undefined
    ): ParserOutput<RonToken, TupleStruct<Literal, Value>> => {
        let parser = apply(
            seq(
                str<RonToken>(name),
                tok(RonToken.LeftParenthesis),
                kleft(fieldsParser(fields), opt(comma)),
                tok(RonToken.RightParenthesis)
            ),
            (value) => {
                return {
                    name,
                    value: value[2],
                };
            }
        );
        return parser.parse(token);
    };
}

function fieldsParser<Value extends any[]>(
    fields: FieldsArray<Value>
): Parser<RonToken, Value> {
    let [first, ...rest] = fields;
    if (rest.length == 0) {
        return apply({ parse: first }, (value) => [value]) as Parser<
            RonToken,
            Value
        >;
    }

    return apply(
        seq({ parse: first }, kright(comma, fieldsParser(rest))),
        (value) => [value[0], ...value[1]]
    ) as Parser<RonToken, Value>;
}

type FieldsArray<V extends any[]> = {
    [Key in keyof V]: RonDecoder<V[Key]>;
};

export type TupleStruct<Literal extends string, Value extends any[]> = {
    name: StringLiteral<Literal>;
    value: Value;
};

export type StringLiteral<T extends string> = string extends T ? never : T;
