import Box from "@mui/material/Box";
import { Fab, List, ListItem, ListItemText, TextField } from "@mui/material";
import { Send } from "@mui/icons-material";
import { FormEvent, useRef } from "react";

export function Chat() {
    return (
        <>
            <Box sx={{ flex: 1 }}>
                <List>
                    <ListItem key="1" disablePadding>
                        <ListItemText primary="hi1" />
                    </ListItem>
                    <ListItem key="2" disablePadding>
                        <ListItemText primary="hi2" />
                    </ListItem>
                </List>
            </Box>
            <SendMessage />
        </>
    );
}

function SendMessage() {
    const input = useRef<HTMLInputElement>(null);

    const onSubmit = (e: FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        const message = input.current?.value;

        console.log(message); // TODO
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
