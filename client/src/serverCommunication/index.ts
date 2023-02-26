import { decode, ronEnum, string, tupleStruct } from '../ron';

const messageDecoder = ronEnum(tupleStruct('Success', string), tupleStruct('Error', string));

export function decodeMessage(message: string): string {
    const result = decode(message, messageDecoder);
    if (!result.success) {
        return 'Could not parse server response: ' + result.error;
    }
    if (result.value.name === 'Success') {
        return result.value.value[0];
    }
    return 'The server returned an error: ' + result.value.value[0];
}
