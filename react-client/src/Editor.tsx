import useWebSocket from "react-use-websocket";
import {useState} from "react";

type EditorState = {
    viewContent: string;
};

const Editor = () => {
    const [state, setState] = useState<EditorState>({viewContent: ''});

    useWebSocket(
        'ws://127.0.0.1:8080',
        {
            share: true,
            onMessage: (event) => setState({viewContent: event.data})
        }
    )

    return <><p>Boah!</p><p>{state.viewContent}</p></>;
};

export default Editor;