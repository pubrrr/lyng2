import React from "react";
import Loading from "./Loading";
import Editor, { websocketUrl } from "./Editor";
import LostConnection from "./LostConnection";
import useWebSocket, { ReadyState } from "react-use-websocket";
import { Link, Route, Routes } from "react-router-dom";
import { Chat } from "./chat";

function App() {
    return (
        <Routes>
            <Route path="/" element={<Menu />} />
            <Route path="lyng" element={<LyngApp />} />
            <Route path="chat" element={<Chat />} />
        </Routes>
    );
}

function Menu() {
    console.log("Menu");
    return (
        <>
            <Link to={"lyng"}>Lyng</Link>
            <Link to={"chat"}>Chat</Link>
        </>
    );
}

function LyngApp() {
    const { readyState } = useWebSocket(websocketUrl, { share: true });
    console.log(readyState);
    switch (readyState) {
        case ReadyState.CONNECTING:
        case ReadyState.UNINSTANTIATED:
            return <Loading />;
        case ReadyState.CLOSED:
        case ReadyState.CLOSING:
            return <LostConnection />;
        case ReadyState.OPEN:
            return <Editor />;
    }
}

export default App;
