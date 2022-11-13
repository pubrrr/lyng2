import { FormEvent, useRef } from "react";
import { useRegisterMutation } from "./gql-types";
import Box from "@mui/material/Box";
import { CircularProgress, Fab, InputAdornment, TextField, Typography } from "@mui/material";
import { AccountCircle, Send } from "@mui/icons-material";
import CheckCircleIcon from "@mui/icons-material/CheckCircle";

export function Register(props: { onSuccess: () => void }) {
    const input = useRef<HTMLInputElement>(null);
    let [register, { data, loading, error }] = useRegisterMutation();

    if (data) {
        props.onSuccess();
        return <CheckCircleIcon />;
    }
    if (loading) {
        return <CircularProgress />;
    }
    if (error) {
        return <Typography>Ohoh: {error.message}</Typography>;
    }

    let onSubmit = (e: FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        const name = input.current?.value;

        if (name === undefined) {
            return;
        }
        register({
            variables: {
                name,
            },
        });
    };

    return (
        <Box component="form" onSubmit={onSubmit} sx={{ mt: 1 }}>
            <TextField
                label="Enter your name"
                id="userName"
                inputRef={input}
                InputProps={{
                    startAdornment: (
                        <InputAdornment position="start">
                            <AccountCircle />
                        </InputAdornment>
                    ),
                }}
            />
            <Fab color="primary" type={"submit"}>
                <Send />
            </Fab>
        </Box>
    );
}
