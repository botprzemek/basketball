import { createServer, IncomingMessage, ServerResponse } from "node:http";

createServer((_request: IncomingMessage, response: ServerResponse): void => {
		response
			.setHeader("Content-Type", "application/json")
			.writeHead(200)
			.end(JSON.stringify([
				{
					team_id: 1,
					name: "Trae",
					lastname: "Young",
					nationality: "USA",
					number: 11,
					height: 185,
					weight: 82,
					wingspan: 191,
					position: "PG",
					birth_date: new Date("1998-09-19"),
					starter: true,
				},
			]));
	})
	.listen(3000, "0.0.0.0", (): void => {
		console.log("  Listening on http://[::]:3000");
	});