import { Schema, model, models } from "mongoose";

const blogSchema = new Schema({
	title: {
		type: String,
		require: [true, "Pues si no... si titulo..."],
		unique: true,
		trim: true // quita espacios adicionales al pricipio y al final de String

	},
	author: String,
	body: { type: String, require: [true, "Men! seriedad"], trim: true },
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

export default models.Blog || model("Blog", blogSchema)