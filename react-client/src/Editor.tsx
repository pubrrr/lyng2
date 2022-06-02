import useWebSocket from "react-use-websocket";
import { ChangeEvent, Dispatch, FormEventHandler, SetStateAction, useState } from "react";
import { decodeMessage } from "./serverCommunication";

type EditorState = {
    editorContent: string;
    viewContent: string;
};

export const websocketUrl = "ws://127.0.0.1:8080";

const Editor = () => {
    const [state, setState] = useState<EditorState>({ editorContent: "", viewContent: "" });

    const { sendMessage } = useWebSocket(websocketUrl, {
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
            <p data-testid="view">{highlightSyntax(state.viewContent)}</p>
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

function highlightSyntax(content: string): JSX.Element {
    const highlighted = content
        .split(/([\^+\-*/()])/g)
        .filter((part) => part.length > 0)
        .map((part, index) => {
            if (["+", "-", "*", "/", "^"].includes(part)) {
                return (
                    <span key={index} style={{ color: "violet" }}>
                        {part}
                    </span>
                );
            }
            if (["(", ")"].includes(part)) {
                return (
                    <span key={index} style={{ color: "aquamarine" }}>
                        {part}
                    </span>
                );
            }
            return <span key={index}>{part}</span>;
        });
    return <>{highlighted}</>;
}

export default Editor;
