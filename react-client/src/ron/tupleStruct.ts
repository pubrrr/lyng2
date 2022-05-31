import { apply, kleft, kright, opt, Parser, ParserOutput, seq, str, tok, Token } from "typescript-parsec";
import { RonDecoder, RonToken, StringLiteral } from "./common";

const comma = seq(opt(tok(RonToken.Whitespace)), str<RonToken>(","), opt(tok(RonToken.Whitespace)));

export type TupleStruct<Literal extends string, Value extends any[]> = {
    name: StringLiteral<Literal>;
    value: Value;
};

export function tupleStruct<Literal extends string, Value extends [...any[]]>(
    name: StringLiteral<Literal>,
    ...fields: FieldDecoders<Value>
): RonDecoder<TupleStruct<Literal, Value>> {
    return (token: Token<RonToken> | undefined): ParserOutput<RonToken, TupleStruct<Literal, Value>> => {
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

function fieldsParser<Value extends [...any[]]>(fields: FieldDecoders<Value>): Parser<RonToken, Value> {
    let [first, ...rest] = fields;
    if (rest.length == 0) {
        return apply({ parse: first }, (value) => [value]) as Parser<RonToken, Value>;
    }

    return apply(seq({ parse: first }, kright(comma, fieldsParser(rest))), (value) => [
        value[0],
        ...value[1],
    ]) as Parser<RonToken, Value>;
}

type FieldDecoders<V extends [...any[]]> = {
    [Key in keyof V]: RonDecoder<V[Key]>;
};
