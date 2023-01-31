import { decodeMessage } from "./index";

describe("decode server messages", () => {
    test("should decode Success messages", () => {
        let result = decodeMessage('Success("success message")');

        expect(result).toBe("success message");
    });

    test("should decode Error messages", () => {
        let result = decodeMessage('Error("error message")');

        expect(result).toBe("The server returned an error: error message");
    });
});
