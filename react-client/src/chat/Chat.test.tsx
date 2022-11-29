import React from "react";
import { act, fireEvent, render, screen } from "@testing-library/react";
import { Chat } from "./Chat";
import { useSendMessageMutation, useSubscribeToChatMessagesSubscription } from "./gql-types";

jest.mock("./gql-types");
const mockUseSendMessageMutation = useSendMessageMutation as jest.Mock;
const mockSendMessage = jest.fn();
const mockUseSubscribeToChatMessagesSubscription =
    useSubscribeToChatMessagesSubscription as jest.Mock;

beforeEach(() => {
    mockUseSendMessageMutation.mockReset();
    mockSendMessage.mockReset();
    mockUseSubscribeToChatMessagesSubscription.mockReset();

    window.HTMLElement.prototype.scrollIntoView = () => {};
    mockUseSendMessageMutation.mockReturnValue([mockSendMessage]);
    mockSendMessage.mockResolvedValue({});
});

const incomingMessage1 = "incoming message1";
const subscriptionsMessage1 = {
    user: { id: "User#1", name: "user name" },
    message: incomingMessage1,
    date: "2022-11-28T23:30:00.000000000+01:00",
};
const incomingMessage2 = "incoming message2";
const subscriptionsMessage2 = {
    user: { id: "User#1", name: "user name" },
    message: incomingMessage2,
    date: "2022-11-28T23:30:00.000000000+01:00",
};

test("show sent message", () => {
    const input = "my input";

    render(<Chat />);

    const inputField = screen.getByRole("textbox");
    fireEvent.input(inputField, { target: { value: input } });

    const submitButton = screen.getByRole("button");
    fireEvent.click(submitButton);

    const message = screen.getByText(input);
    expect(message).toBeInTheDocument();
    expect(mockSendMessage).toHaveBeenCalledWith({ variables: { message: input } });
});

test("show incoming message", () => {
    render(<Chat />);

    act(() => {
        mockUseSubscribeToChatMessagesSubscription.mock.calls[0][0].onData({
            data: { data: { getNewMessages: subscriptionsMessage1 } },
        });
    });

    const message = screen.getByText(incomingMessage1);
    expect(message).toBeInTheDocument();
});

test("show multiple incoming messages", () => {
    render(<Chat />);

    act(() => {
        mockUseSubscribeToChatMessagesSubscription.mock.calls[0][0].onData({
            data: { data: { getNewMessages: subscriptionsMessage1 } },
        });
    });

    expect(screen.getByText(incomingMessage1)).toBeInTheDocument();
    expect(screen.queryByText(incomingMessage2)).not.toBeInTheDocument();

    act(() => {
        mockUseSubscribeToChatMessagesSubscription.mock.calls[0][0].onData({
            data: { data: { getNewMessages: subscriptionsMessage2 } },
        });
    });

    render(<Chat />);

    expect(screen.getByText(incomingMessage1)).toBeInTheDocument();
    expect(screen.getByText(incomingMessage2)).toBeInTheDocument();
});
