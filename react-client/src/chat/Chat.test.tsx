import React from "react";
import { fireEvent, render, screen } from "@testing-library/react";
import { Chat } from "./Chat";

test("show sent message", () => {
    window.HTMLElement.prototype.scrollIntoView = () => {};
    render(<Chat />);

    const inputField = screen.getByRole("textbox");
    fireEvent.input(inputField, { target: { value: "my input" } });

    const submitButton = screen.getByRole("button");
    fireEvent.click(submitButton);

    const message = screen.getByText("my input");
    expect(message).toBeInTheDocument();
});
