// GitHub repos API endpoint for Cloudflare Pages Functions
export async function onRequest(context) {
    const { env } = context;
    
    const username = env.GITHUB_USERNAME || 'System-End';
    const url = `https://api.github.com/users/${username}/repos?sort=updated&per_page=10`;
    
    try {
        const response = await fetch(url, {
            headers: {
                'User-Agent': 'personal-site-cloudflare',
                'Accept': 'application/vnd.github.v3+json'
            }
        });
        
        if (!response.ok) {
            return new Response(JSON.stringify({
                error: 'GitHub API error'
            }), {
                status: 502,
                headers: { 
                    'Content-Type': 'application/json',
                    'Access-Control-Allow-Origin': '*'
                }
            });
        }
        
        const repos = await response.json();
        
        const formattedRepos = repos.map(repo => ({
            name: repo.name,
            description: repo.description || '',
            url: repo.html_url,
            language: repo.language || 'Unknown',
            stars: repo.stargazers_count,
            forks: repo.forks_count,
            updated_at: repo.updated_at,
            topics: repo.topics || []
        }));
        
        return new Response(JSON.stringify(formattedRepos), {
            headers: { 
                'Content-Type': 'application/json',
                'Access-Control-Allow-Origin': '*',
                'Cache-Control': 'max-age=300'
            }
        });
        
    } catch (error) {
        return new Response(JSON.stringify({
            error: 'Unable to reach GitHub'
        }), {
            status: 503,
            headers: { 
                'Content-Type': 'application/json',
                'Access-Control-Allow-Origin': '*'
            }
        });
    }
}
