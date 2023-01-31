import React from "react";
import { act, render, screen } from "@testing-library/react";
import { useGetNewUsersSubscription, useGetUsersQuery } from "./gql-types";
import { Users } from "./Users";

jest.mock("./gql-types");
const mockUseGetUsersQuery = useGetUsersQuery as jest.Mock;
const mockUseGetNewUsersSubscription = useGetNewUsersSubscription as jest.Mock;

beforeEach(() => {
    mockUseGetUsersQuery.mockReset();
    mockUseGetNewUsersSubscription.mockReset();
});

const userName1 = "userName1";
const user1 = {
    id: "userId1",
    name: userName1,
};
const userName2 = "userName2";
const user2 = {
    id: "userId2",
    name: userName2,
};

test("shows users from initial query", () => {
    render(<Users />);

    act(() => {
        mockUseGetUsersQuery.mock.calls[0][0].onCompleted({
            getUsers: [user1, user2],
        });
    });

    expect(screen.getByText(userName1)).toBeInTheDocument();
    expect(screen.getByText(userName2)).toBeInTheDocument();
});

test("shows user sent later from subscription", () => {
    render(<Users />);

    act(() => {
        mockUseGetNewUsersSubscription.mock.calls[0][0].onData({
            data: { data: { getNewUsers: user1 } },
        });
    });

    expect(screen.getByText(userName1)).toBeInTheDocument();
});
