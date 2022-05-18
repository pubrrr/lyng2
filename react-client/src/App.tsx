import React, { useState } from "react";
import Loading from "./Loading";
import LoadingFailed from "./LoadingFailed";
import Editor from "./Editor";
import LostConnection from "./LostConnection";
import useWebSocket from "react-use-websocket";

type State = "connected" | "loading" | "failed loading" | "lost connection";

function App(): JSX.Element {
    const [state, setState] = useState<State>("loading");

    useWebSocket("ws://127.0.0.1:8080", {
        share: true,
        onOpen: () => setState("connected"),
        onError: () => setState("failed loading"),
        onClose: () => setState("lost connection"),
    });

    switch (state) {
        case "loading":
            return <Loading />;
        case "failed loading":
            return <LoadingFailed />;
        case "lost connection":
            return <LostConnection />;
        case "connected":
            return <Editor />;
    }
}

export default App;
