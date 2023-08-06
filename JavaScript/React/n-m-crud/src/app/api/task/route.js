import { NextResponse } from "next/server";
import { connectDB } from "@/utils/mongoose";
import Blog from "@/models/task";

export function GET() {
	return NextResponse.json({
		mensaje: "geting task..."
	})
}
export async function POST(request) {
	try {
		const data = await request.json()
		const task = new Blog(data)
		const savetask = await task.save()
		// connectDB()
		return NextResponse.json({
			mensaje: `insert task...`,
			action: "Insert task",
			method: "POST",
			res: "saccess",
			debbug: { req_body: data, created_model: task, saved_model: savetask }
		})
	} catch (error) {
		return NextResponse.json(error.message, {
			status: 400,
			error
		})
	}

}