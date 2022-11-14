import Box from "@mui/material/Box";
import { Fab, List, ListItem, ListItemText, TextField } from "@mui/material";
import { Send } from "@mui/icons-material";
import { FormEvent, useRef, useState } from "react";

type Message = { message: string; time: Date };

export function Chat() {
    const [messages, setMessages] = useState<Message[]>([]);

    const onSendMessage = (message: string) => {
        setMessages((messages) => {
            let newMessages = new Array(...messages);
            newMessages.push({
                message,
                time: new Date(),
            });
            return newMessages;
        });
    };

    return (
        <>
            <Box sx={{ flex: 1 }}>
                <List>
                    {messages.map((message) => (
                        <ListItem key={message.time.toUTCString()} disablePadding>
                            <ListItemText primary={message.message} />
                        </ListItem>
                    ))}
                </List>
            </Box>
            <SendMessage onSendMessage={onSendMessage} />
        </>
    );
}

function SendMessage({ onSendMessage }: { onSendMessage: (message: string) => void }) {
    const input = useRef<HTMLInputElement>(null);

    const onSubmit = (e: FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        if (input.current?.value) {
            onSendMessage(input.current.value);
        }
    };

    return (
        <Box
            component="form"
            onSubmit={onSubmit}
            sx={{ mt: 1, display: "flex", justifyContent: "center", flexFlow: "no-wrap" }}
        >
            <TextField
                placeholder="Enter your message"
                id="messageInput"
                inputRef={input}
                sx={{ flex: 1 }}
            />
            <Fab color="primary" type={"submit"} sx={{ ml: 1 }}>
                <Send />
            </Fab>
        </Box>
    );
}
