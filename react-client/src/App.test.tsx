import useWebSocket from "react-use-websocket";
import React from 'react';
import {render, screen} from '@testing-library/react';
import App from './App';
import {act} from "react-dom/test-utils";
import Editor from "./Editor";
import LoadingFailed from "./LoadingFailed";
import LostConnection from "./LostConnection";

jest.mock("react-use-websocket")
jest.mock("./Editor")
jest.mock("./LoadingFailed")
jest.mock("./LostConnection")

const mockUseWebSocket = useWebSocket as jest.Mock;

test('initially renders loading screen', () => {
    render(<App/>);

    const laodingElement = screen.getByText(/Loading/i);
    expect(laodingElement).toBeInTheDocument();
});

test('initializes Websocket', () => {
    render(<App/>);

    expect(mockUseWebSocket).toBeCalled();
});

test('changes to editor when websocket connected', () => {
    render(<App/>);

    act(() => {
        mockUseWebSocket.mock.calls[0][1].onOpen()
    });
    expect(Editor).toBeCalled();
});

test('changes to editor when websocket connected', () => {
    render(<App/>);

    act(() => {
        mockUseWebSocket.mock.calls[0][1].onError()
    });
    expect(LoadingFailed).toBeCalled();
});

test('changes to editor when websocket connected', () => {
    render(<App/>);

    act(() => {
        mockUseWebSocket.mock.calls[0][1].onClose()
    });
    expect(LostConnection).toBeCalled();
});
