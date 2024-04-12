const http = require('http');
const fs = require('fs');

// Read and execute the JavaScript file on server startup
fs.readFile('script.js', 'utf8', (err, data) => {
    if (err) {
        console.error("Error reading file:", err);
        return;
    }

    // Execute the JavaScript code
    eval(data);
});

// Create a basic server
const server = http.createServer((req, res) => {
    res.writeHead(200, { 'Content-Type': 'text/plain' });
    res.end('Server is running');
});

const PORT = 3000;
server.listen(PORT, () => {
    console.log(`Server is running on http://localhost:${PORT}`);
});
