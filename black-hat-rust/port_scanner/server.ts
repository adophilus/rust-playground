const subdomains = await Bun.file("./cached_response.json").text();

const server = Bun.serve({
	port: 3000,
	fetch(request) {
		console.log(subdomains);
		return new Response(subdomains);
	},
});

console.log(`Listening on ${server.url}`);
