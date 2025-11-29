// Last.fm API endpoint for Cloudflare Pages Functions
export async function onRequest(context) {
    const { env } = context;
    
    const apiKey = env.LASTFM_API_KEY;
    const username = env.LASTFM_USERNAME || 'EndofTimee';
    
    if (!apiKey) {
        return new Response(JSON.stringify({
            current_track: null,
            error: 'Last.fm API key not configured'
        }), {
            headers: { 
                'Content-Type': 'application/json',
                'Access-Control-Allow-Origin': '*'
            }
        });
    }
    
    const url = `https://ws.audioscrobbler.com/2.0/?method=user.getrecenttracks&user=${username}&api_key=${apiKey}&format=json&limit=1`;
    
    try {
        const response = await fetch(url, {
            headers: {
                'User-Agent': 'personal-site-cloudflare'
            }
        });
        
        if (!response.ok) {
            return new Response(JSON.stringify({
                current_track: null,
                error: 'Last.fm API error'
            }), {
                headers: { 
                    'Content-Type': 'application/json',
                    'Access-Control-Allow-Origin': '*'
                }
            });
        }
        
        const data = await response.json();
        
        if (!data.recenttracks?.track?.length) {
            return new Response(JSON.stringify({
                current_track: null,
                error: null
            }), {
                headers: { 
                    'Content-Type': 'application/json',
                    'Access-Control-Allow-Origin': '*'
                }
            });
        }
        
        const track = data.recenttracks.track[0];
        const isPlaying = track['@attr']?.nowplaying === 'true';
        
        // Get large image or fallback
        const largeImage = track.image?.find(img => img.size === 'large');
        const image = largeImage?.['#text'] || track.image?.[track.image.length - 1]?.['#text'] || '';
        
        const currentTrack = {
            name: track.name,
            artist: track.artist['#text'],
            image: image,
            is_playing: isPlaying
        };
        
        return new Response(JSON.stringify({
            current_track: currentTrack,
            error: null
        }), {
            headers: { 
                'Content-Type': 'application/json',
                'Access-Control-Allow-Origin': '*',
                'Cache-Control': 'max-age=30'
            }
        });
        
    } catch (error) {
        return new Response(JSON.stringify({
            current_track: null,
            error: 'Unable to reach Last.fm'
        }), {
            headers: { 
                'Content-Type': 'application/json',
                'Access-Control-Allow-Origin': '*'
            }
        });
    }
}
