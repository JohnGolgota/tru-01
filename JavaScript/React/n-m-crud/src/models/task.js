import { Schema, model, models } from "mongoose";

const taskSchema = new Schema({
	title: {
		type: String,
		required: [true, 'a?'],
		unique: true,
		trim: true // quita espacios adicionales al pricipio y al final de String
	},
	author: String,
	body: {
		type: String,
		required: [true, "Men! seriedad"],
		trim: true
	},
	comments: [{ body: String, date: Date }],
	date: { type: Date, default: Date.now },
	hidden: Boolean,
	meta: {
		votes: Number,
		favs: Number
	}
}, {
	timestamps: true
});

export default models.Task || model("Task", taskSchema)