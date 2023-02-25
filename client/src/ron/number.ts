import { ParseResult as ParsecParseResult, ParserOutput, Token } from 'typescript-parsec';
import { RonToken } from './common';
import { string } from './string';

type ParseResult<T> = ParsecParseResult<RonToken, T>;

export function number(token: Token<RonToken> | undefined): ParserOutput<RonToken, number> {
    const stringResult = string(token);

    if (!stringResult.successful) {
        return stringResult;
    }

    const parsedNumberCandidates = stringResult.candidates.map(tryToParseToNumber);

    let unparseableString = parsedNumberCandidates.find(
        (value: ParseResult<string> | ParseResult<number>) => typeof value.result == 'string'
    );
    if (unparseableString !== undefined) {
        return createErrorFor(unparseableString as ParseResult<string>);
    }

    return {
        ...stringResult,
        candidates: parsedNumberCandidates as ParseResult<number>[],
    };
}

function tryToParseToNumber(candidate: ParseResult<string>): ParseResult<string> | ParseResult<number> {
    if (!looksLikeANumber(candidate)) {
        return candidate;
    }

    const number = parseFloat(candidate.result);
    if (!isNaN(number)) {
        return {
            ...candidate,
            result: number,
        };
    }

    return candidate;
}

function looksLikeANumber(value: ParseResult<string>) {
    return /^\d+$/.test(value.result) || /^\d*\.\d*$/.test(value.result);
}

function createErrorFor(unparsableString: ParseResult<string>): ParserOutput<RonToken, number> {
    return {
        successful: false,
        error: {
            kind: 'Error',
            pos: undefined,
            message: 'could not parse ' + unparsableString.result + ' to a number',
        },
    };
}
