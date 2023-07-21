import { createSlice, type PayloadAction } from "@reduxjs/toolkit";
export type UserId = string;
export interface contact {
	name: string;
	email: string;
	github: string;
}
export interface contactsWithId extends contact {
	id: UserId;
}

const DEFAULT_STATE = [
	{
		id: "1",
		name: "Erika",
		email: "a@a.a",
		github: "JohnGolgota",
	},
	{
		id: "2",
		name: "Lana",
		email: "b@b.b",
		github: "DuvanArwenLazar",
	},
	{
		id: "3",
		name: "J",
		email: "c@c.c",
		github: "Kolozuz",
	},
];

const initialState: contactsWithId[] = (() => {
	const persistedState = localStorage.getItem("__redux__state__");
	if (persistedState) {
		return JSON.parse(persistedState).contacts;
	}
	return DEFAULT_STATE;
})();
export const contactsSlice = createSlice({
	name: "contacts",
	initialState,
	reducers: {
		deleteContactByIdUno: (
			state,
			action: { type: string; payload: UserId },
		) => {
			const id = action.payload;
			return state.filter((user) => user.id !== id);
		},
		deleteContactById: (state, action: PayloadAction<UserId>) => {
			const id = action.payload;
			return state.filter((user) => user.id !== id);
		},
		addNewContact: (state, action: PayloadAction<contact>) => {
			const id = crypto.randomUUID();
			return [...state, { id, ...action.payload }];
		},
		rollBackDeleteContact: (state, action: PayloadAction<contactsWithId>) => {
			const isContactAlredy = state.some(
				(contact) => contact.id === action.payload.id,
			);
			if (!isContactAlredy) {
				return [...state, action.payload];
			}
		},
	},
});

export default contactsSlice;

export const { deleteContactById, addNewContact, rollBackDeleteContact } =
	contactsSlice.actions;
