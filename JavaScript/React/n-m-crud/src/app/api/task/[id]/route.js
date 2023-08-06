import { NextResponse } from "next/server";

export function POST(request, { params }) {
	return NextResponse.json({
		mensaje: `insert task... ${params.id}`,
		debbug: params
	})
}
export function GET(request, { params }) {
	return NextResponse.json({
		mensaje: `geting task... ${params.id}`,
		debbug: params
	})
}
export function PUT(request, { params }) {
	return NextResponse.json({
		mensaje: `update task... ${params.id}`,
		debbug: params
	})
}
export function DELETE(request, { params }) {
	return NextResponse.json({
		mensaje: `delete task... ${params.id}`,
		debbug: params
	})
}