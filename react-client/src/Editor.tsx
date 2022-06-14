import useWebSocket from "react-use-websocket";
import { ChangeEvent, Dispatch, FormEventHandler, SetStateAction, useState } from "react";
import { decodeMessage } from "./serverCommunication";

type EditorState = {
    editorContent: string;
    viewContent: string;
};

const Editor = () => {
    const [state, setState] = useState<EditorState>({ editorContent: "", viewContent: "" });

    const { sendMessage } = useWebSocket("ws://127.0.0.1:8080", {
        share: true,
        onMessage: (event) =>
            setState((prevState) => {
                return { ...prevState, viewContent: decodeMessage(event.data) };
            }),
    });

    const onClick = () => sendMessage(state.editorContent);

    return (
        <>
            <button onClick={onClick}>Send</button>
            <div data-testid="input" contentEditable={true} onInput={setInput(setState)}></div>
            <p data-testid="view">{state.viewContent}</p>
        </>
    );
};

function setInput(setState: Dispatch<SetStateAction<EditorState>>): FormEventHandler {
    return (event: ChangeEvent<HTMLDivElement>) => {
        setState((prevState) => {
            return {
                ...prevState,
                editorContent: event.target.innerText,
            };
        });
    };
}

export default Editor;
