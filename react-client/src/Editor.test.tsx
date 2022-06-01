import { fireEvent, render, screen } from "@testing-library/react";
import Editor from "./Editor";
import { act } from "react-dom/test-utils";
import useWebSocket from "react-use-websocket";
import { decodeMessage } from "./serverCommunication";

jest.mock("react-use-websocket");
const mockUseWebSocket = useWebSocket as jest.Mock;
const sendMessageMock = jest.fn();

jest.mock("./serverCommunication");
const decodeMessageMock = decodeMessage as jest.Mock;

describe("the Editor", () => {
    beforeEach(() => {
        sendMessageMock.mockReset();
        decodeMessageMock.mockReset();
        mockUseWebSocket.mockReturnValue({ sendMessage: sendMessageMock });
    });

    test("should send the input data to the server", () => {
        render(<Editor />);

        const inputField = screen.getByTestId("input");
        fireEvent.input(inputField, { target: { innerText: "my input" } });
        fireEvent.click(screen.getByText("Send"));

        expect(sendMessageMock).toBeCalledWith("my input");
    });

    test("should put data received from the server to the view", () => {
        decodeMessageMock.mockReturnValue("decoded message");

        render(<Editor />);

        act(() => {
            mockUseWebSocket.mock.calls[0][1].onMessage({ data: "from the server" });
        });

        const view = screen.getByTestId("view");
        expect(view).toHaveTextContent("decoded message");
        expect(decodeMessageMock).toHaveBeenCalledWith("from the server");
    });
});
