import { render, screen } from '@testing-library/react';
import Editor from './Editor';
import useWebSocket from 'react-use-websocket';
import { decodeMessage } from './serverCommunication';

jest.mock('react-use-websocket');
const mockUseWebSocket = useWebSocket as jest.Mock;
const sendMessageMock = jest.fn();

jest.mock('./serverCommunication');
const decodeMessageMock = decodeMessage as jest.Mock;

describe('the Editor', () => {
    beforeEach(() => {
        sendMessageMock.mockReset();
        decodeMessageMock.mockReset();
        mockUseWebSocket.mockReturnValue({ sendMessage: sendMessageMock });
    });

    test('should display the initial text', () => {
        render(<Editor />);

        const inputField = screen.getAllByRole('textbox')[0];

        expect(inputField).toHaveTextContent('1+2+3+4');
    });
});
