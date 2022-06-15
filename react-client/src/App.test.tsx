import useWebSocket from "react-use-websocket";
import React from "react";
import { render, screen } from "@testing-library/react";
import App from "./App";
import { act } from "react-dom/test-utils";
import Editor from "./Editor";
import LoadingFailed from "./LoadingFailed";
import LostConnection from "./LostConnection";
import { BrowserRouter } from "react-router-dom";

jest.mock("react-use-websocket");
jest.mock("./Editor");
jest.mock("./LoadingFailed");
jest.mock("./LostConnection");

const useWebSocketMock = useWebSocket as jest.Mock;

function renderApp() {
    render(
        <BrowserRouter>
            <App />
        </BrowserRouter>
    );
}

test("initially renders loading screen", () => {
    renderApp();

    const laodingElement = screen.getByText(/Loading/i);
    expect(laodingElement).toBeInTheDocument();
});

test("initializes Websocket", () => {
    renderApp();

    expect(useWebSocketMock).toBeCalled();
});

test("changes to editor when websocket connected", () => {
    renderApp();

    act(() => {
        useWebSocketMock.mock.calls[0][1].onOpen();
    });
    expect(Editor).toBeCalled();
});

test("changes to loading failed when websocket connection fails", () => {
    renderApp();

    act(() => {
        useWebSocketMock.mock.calls[0][1].onError();
    });
    expect(LoadingFailed).toBeCalled();
});

test("changes to lost connection when websocket connection terminates", () => {
    renderApp();

    act(() => {
        useWebSocketMock.mock.calls[0][1].onClose();
    });
    expect(LostConnection).toBeCalled();
});
