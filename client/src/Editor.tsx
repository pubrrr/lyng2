import useWebSocket from 'react-use-websocket';
import { PropsWithChildren, useState } from 'react';
import { decodeMessage } from './serverCommunication';
import CodeMirror from '@uiw/react-codemirror';
import { Box } from '@mui/material';

type EditorState = {
    editorContent: string;
    viewContent: string;
};

export const websocketUrl = 'ws://' + window.location.host + '/api/math';

const Editor = () => {
    const [state, setState] = useState<EditorState>({ editorContent: '', viewContent: '' });

    const { sendMessage } = useWebSocket(websocketUrl, {
        share: true,
        onMessage: (event) =>
            setState((prevState) => {
                return { ...prevState, viewContent: decodeMessage(event.data) };
            }),
    });

    return (
        <Box sx={{ p: 3, display: 'flex' }}>
            <EditorContainer>
                <CodeMirror value='1+2+3+4' height='90vh' onChange={(message: string) => sendMessage(message)} />
            </EditorContainer>
            <EditorContainer>
                <CodeMirror value={state.viewContent} height='90vh' editable={false} />
            </EditorContainer>
        </Box>
    );
};

function EditorContainer(props: PropsWithChildren) {
    return <Box sx={{ width: '40%', flex: 1, m: 1 }}>{props.children}</Box>;
}

export default Editor;
