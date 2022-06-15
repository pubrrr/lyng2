import React from "react";
import Loading from "./Loading";
import LoadingFailed from "./LoadingFailed";
import Editor, { websocketUrl } from "./Editor";
import LostConnection from "./LostConnection";
import useWebSocket from "react-use-websocket";
import { Route, Routes, useNavigate } from "react-router-dom";

function App(): JSX.Element {
    const navigate = useNavigate();

    useWebSocket(websocketUrl, {
        share: true,
        onOpen: () => navigate("editor"),
        onError: () => navigate("connectionFailed"),
        onClose: () => navigate("lostConnection"),
    });

    return (
        <Routes>
            <Route path="/" element={<Loading />} />
            <Route path="connectionFailed" element={<LoadingFailed />} />
            <Route path="lostConnection" element={<LostConnection />} />
            <Route path="editor" element={<Editor />} />
        </Routes>
    );
}

export default App;
