import React from "react";
import Loading from "./Loading";
import Editor, { websocketUrl } from "./Editor";
import LostConnection from "./LostConnection";
import useWebSocket, { ReadyState } from "react-use-websocket";
import { Link, Route, Routes } from "react-router-dom";
import { ChatApp } from "./chat";
import { Box, Card, CardActionArea, CardContent, Typography } from "@mui/material";

function App() {
    return (
        <Routes>
            <Route path="/" element={<Menu />} />
            <Route path="lyng" element={<LyngApp />} />
            <Route path="chat" element={<ChatApp />} />
        </Routes>
    );
}

function Menu() {
    return (
        <Box
            sx={{
                display: "flex",
                justifyContent: "center",
                alignItems: "center",
                height: "100vh",
            }}
        >
            <LinkCard to={"lyng"} text="Lyng" />
            <LinkCard to={"chat"} text="Chat" />
        </Box>
    );
}

function LinkCard(props: { to: string; text: string }) {
    return (
        <Card
            sx={{
                m: 1,
            }}
        >
            <CardActionArea component={Link} to={props.to}>
                <CardContent
                    sx={{
                        height: "100%",
                        margin: "auto",
                        display: "flex",
                        justifyContent: "center",
                        alignItems: "center",
                        minWidth: "10em",
                        minHeight: "5em",
                    }}
                >
                    <Typography variant="h5">{props.text}</Typography>
                </CardContent>
            </CardActionArea>
        </Card>
    );
}

function LyngApp() {
    const { readyState } = useWebSocket(websocketUrl, { share: true });
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
