import { ParserOutput, Token } from 'typescript-parsec';

export enum RonToken {
    Identifier,
    String,
    LeftParenthesis,
    RightParenthesis,
    Whitespace,
    Comma,
}

export interface RonDecoder<V> {
    (token: Token<RonToken> | undefined): ParserOutput<RonToken, V>;
}

export type StringLiteral<T extends string> = string extends T ? never : T;
