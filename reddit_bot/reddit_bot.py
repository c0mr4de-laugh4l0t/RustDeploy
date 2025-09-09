
import praw
import subprocess

reddit = praw.Reddit(
    client_id="YOUR_CLIENT_ID",
    client_secret="YOUR_CLIENT_SECRET",
    user_agent="ContainerQuest Bot by u/YOUR_USERNAME",
    username="YOUR_USERNAME",
    password="YOUR_PASSWORD"
)


def run_game_command():
    result = subprocess.run(
        ["cargo", "run", "--", "leaderboard"], 
        capture_output=True, 
        text=True
    )
    return result.stdout.strip()

# 📝 Post to your subreddit
def post_to_reddit():
    output = run_game_command()
    subreddit = reddit.subreddit("ContainerQuest")  # change if different
    title = "📊 Daily ContainerQuest Leaderboard"
    
    subreddit.submit(title, selftext=f"```\n{output}\n```")
    print("✅ Posted to Reddit!")

if __name__ == "__main__":
    post_to_reddit()
