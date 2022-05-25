import {
    apply,
    buildLexer,
    err,
    expectEOF,
    kmid,
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
    DoubleQuote,
}

const RON_LEXER = buildLexer([
    [true, /^[a-zA-Z][a-zA-Z\d]*/g, RonToken.Identifier],
    [true, /^[^")(]*/g, RonToken.String],
    [true, /^\(/g, RonToken.LeftParenthesis],
    [true, /^\)/g, RonToken.RightParenthesis],
    [true, /^"/g, RonToken.DoubleQuote],
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
    const parseResult = expectEOF(decoder(RON_LEXER.parse(input)));

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

export const string = kmid(
    err(tok(RonToken.DoubleQuote), "expected opening double quotes"),
    apply(tok(RonToken.String), (token: Token<RonToken>): string => token.text),
    err(tok(RonToken.DoubleQuote), "expected closing double quotes")
).parse;

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
                fieldsParser(fields),
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
    function parse(
        token: Token<RonToken> | undefined
    ): ParserOutput<RonToken, Value> {
        let [first, ...rest] = fields;

        let result = apply(
            { parse: first as RonDecoder<Value[0]> },
            (value) => [value]
        ).parse(token);
        if (!result.successful) {
            return result;
        }

        if (rest.length == 0) {
            return result as ParserOutput<RonToken, Value>;
        }
        throw "ohoh";
    }

    return { parse };
}

type FieldsArray<V extends any[]> = {
    [Key in keyof V]: RonDecoder<V[Key]>;
};

export type TupleStruct<Literal extends string, Value extends any[]> = {
    name: StringLiteral<Literal>;
    value: Value;
};

export type StringLiteral<T extends string> = string extends T ? never : T;

export interface EnumVariant<Literal extends string, Value> {
    name: StringLiteral<Literal>;
    value: Value;
}
