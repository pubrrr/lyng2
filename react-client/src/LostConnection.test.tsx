import {render, screen} from "@testing-library/react";
import LostConnection from "./LostConnection";

test('renders loading failed screen', () => {
    render(<LostConnection/>);

    const text = screen.getByText(/Lost Connection/i);
    expect(text).toBeInTheDocument();
})