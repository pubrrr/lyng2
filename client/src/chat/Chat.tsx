import Box from '@mui/material/Box';
import { Card, CardContent, Fab, List, ListItem, TextField, Typography } from '@mui/material';
import { Send } from '@mui/icons-material';
import { Dispatch, FormEvent, SetStateAction, useEffect, useRef, useState } from 'react';
import {
    Message as ServerMessage,
    SubscribeToChatMessagesSubscription,
    useSendMessageMutation,
    useSubscribeToChatMessagesSubscription,
} from './gql-types';
import { OnDataOptions } from '@apollo/client';

type Message = { message: string; date: Date };

function useMessages() {
    const [messages, setMessages] = useState<Message[]>([]);
    const [sendMessageMutation] = useSendMessageMutation();
    useSubscribeToChatMessagesSubscription({
        onData: storeSubscriptionMessage(setMessages),
    });

    const sendMessage = (message: string) => {
        setMessages((messages) => [...messages, { message, date: new Date() }]);
        sendMessageMutation({ variables: { message } }).catch(console.log);
    };

    return { messages, sendMessage };
}

function storeSubscriptionMessage(setMessages: Dispatch<SetStateAction<Message[]>>) {
    return (options: OnDataOptions<SubscribeToChatMessagesSubscription>) => {
        let newMessage = options.data?.data?.getNewMessages;
        if (newMessage !== undefined) {
            setMessages((messages) => [...messages, mapMessage(newMessage as ServerMessage)]);
        } else if (options.data?.error !== undefined) {
            console.log(options.data.error);
        }
    };
}

function mapMessage(newMessage: ServerMessage) {
    return {
        message: (newMessage as Message).message,
        date: new Date((newMessage as Message).date),
    };
}

export function Chat() {
    const bottomRef = useRef<HTMLLIElement>(null);
    const { messages, sendMessage } = useMessages();

    useEffect(() => {
        bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
    }, [messages]);

    return (
        <>
            <List sx={{ flex: 1, maxHeight: '100%', overflow: 'auto' }}>
                {messages.map((message, i) => (
                    <MessageListItem message={message} key={i} />
                ))}
                <ListItem key='bottom' ref={bottomRef} sx={{ p: 0 }}></ListItem>
            </List>
            <SendMessage onSendMessage={sendMessage} />
        </>
    );
}

function MessageListItem({ message }: { message: Message }) {
    return (
        <ListItem key={message.date.toUTCString()} sx={{ pr: 2, pl: 2, display: 'flex', justifyContent: 'end', width: null }}>
            <Card>
                <CardContent>
                    <Typography>{message.message}</Typography>
                    <Typography align='right' variant='subtitle2' sx={{ mb: -2.5, mr: -1 }}>
                        {format(message.date)}
                    </Typography>
                </CardContent>
            </Card>
        </ListItem>
    );
}

function format(time: Date) {
    let hours = time.getHours();
    let minutes = time.getMinutes();
    return (hours < 10 ? '0' : '' + hours) + ':' + (minutes < 10 ? '0' : '' + minutes);
}

function SendMessage({ onSendMessage }: { onSendMessage: (message: string) => void }) {
    let input = useRef<HTMLInputElement>(null);

    const onSubmit = (e: FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        if (input.current?.value) {
            onSendMessage(input.current.value);
        }
    };

    return (
        <Box component='form' onSubmit={onSubmit} sx={{ mt: 1, display: 'flex', justifyContent: 'center', flexFlow: 'no-wrap' }}>
            <TextField placeholder='Enter your message' id='messageInput' name='messageInput' inputRef={input} sx={{ flex: 1 }} />
            <Fab color='primary' type={'submit'} sx={{ ml: 1 }}>
                <Send />
            </Fab>
        </Box>
    );
}
