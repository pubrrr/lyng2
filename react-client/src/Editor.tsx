import useWebSocket from "react-use-websocket";
import { ChangeEvent, Dispatch, FormEventHandler, SetStateAction, useState } from "react";
import { decodeMessage } from "./serverCommunication";

type EditorState = {
    editorContent: string;
    viewContent: string;
};

export const websocketUrl = "ws://127.0.0.1:8080/api/math";

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
        <div className="editorContainer">
            <div className="inputContainer">
                <button onClick={onClick}>Send</button>
                <p
                    data-testid="input"
                    className="code input"
                    contentEditable={true}
                    onInput={setInput(setState)}
                ></p>
            </div>
            <p data-testid="view" className="code view">
                {highlightSyntax(state.viewContent)}
            </p>
        </div>
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
