import useWebSocket, { ReadyState } from "react-use-websocket";
import React from "react";
import { fireEvent, render, screen } from "@testing-library/react";
import App from "./App";
import Editor from "./Editor";
import LostConnection from "./LostConnection";
import { MemoryRouter } from "react-router-dom";

jest.mock("react-use-websocket");
const mockUseWebSocket = useWebSocket as jest.Mock;
jest.mock("./Editor");
jest.mock("./LostConnection");

function renderApp() {
    render(
        <MemoryRouter>
            <App />
        </MemoryRouter>
    );
}

function goToEditor() {
    fireEvent.click(screen.getByText("Lyng"));
}

test("initially renders loading screen", () => {
    mockUseWebSocket.mockReturnValue({ readyState: ReadyState.CONNECTING });

    renderApp();
    goToEditor();

    const loadingElement = screen.getByText(/Loading/i);
    expect(loadingElement).toBeInTheDocument();
});

test("changes to editor when websocket connected", () => {
    mockUseWebSocket.mockReturnValue({ readyState: ReadyState.OPEN });

    renderApp();
    goToEditor();

    expect(Editor).toBeCalled();
});

test("changes to lost connection when websocket connection terminates", () => {
    mockUseWebSocket.mockReturnValue({ readyState: ReadyState.CLOSED });

    renderApp();
    goToEditor();

    expect(LostConnection).toBeCalled();
});
