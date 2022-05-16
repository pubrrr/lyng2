import {render, screen} from "@testing-library/react";
import LoadingFailed from "./LoadingFailed";

test('renders loading failed screen', () => {
    render(<LoadingFailed/>);

    const text = screen.getByText(/Loading Failed/i);
    expect(text).toBeInTheDocument();
})