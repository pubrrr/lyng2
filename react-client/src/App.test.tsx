import React from 'react';
import {render, screen} from '@testing-library/react';
import App from './App';

test('initially renders loading screen', () => {
    render(<App/>);

    const laodingElement = screen.getByText(/Loading/i);
    expect(laodingElement).toBeInTheDocument();
});
