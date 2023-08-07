import { NextResponse } from "next/server";
import { connectDB } from "@/utils/mongoose";
import Task from "@/models/task";

export async function GET() {
	try {
		connectDB()
		const tasks = await Task.find()
		return NextResponse.json({
			tasks
		})
	} catch (error) {
		return NextResponse.json({
			error,
			mensaje: "JODERRRRRRRRRRRRRR",
		}, {
			status: 500
		})
	}
}
export async function POST(request) {
	try {
		const data = await request.json()
		const task = new Task(data)
		const savetask = await task.save()
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