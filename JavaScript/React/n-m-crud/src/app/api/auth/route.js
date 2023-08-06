import { NextResponse } from "next/server";
import { connectDB } from "@/utils/mongoose";

export function GET() {
	connectDB()
	return NextResponse.json({
		mensaje: "Me voy a pegar un tiro en la boca"
	})
}