// Health check endpoint for Cloudflare Pages Functions
export async function onRequest(context) {
    return new Response(JSON.stringify({
        status: 'healthy',
        version: '0.9.5-rust',
        platform: 'cloudflare-pages'
    }), {
        headers: { 
            'Content-Type': 'application/json',
            'Access-Control-Allow-Origin': '*'
        }
    });
}
