import { render, screen } from '@testing-library/react';
import Loading from './Loading';

test('renders loading screen', () => {
    render(<Loading />);

    const loadingText = screen.getByText(/Loading/i);
    expect(loadingText).toBeInTheDocument();
});
