import requests
import jinja2

from utils import ascii_progress_bar

STATS_URL = "https://wakatime.com/api/v1/users/lognes/stats"

# Most of YAML is Kubernetes, but I don't really care for it to be shown on my profile.
EXCLUDED_LANGS = ["YAML"]

# Open up the jinja template readme
with open('./template-readme.md', 'r', encoding='utf-8') as readme_in:
    template = jinja2.Template(source = readme_in.read())

req = requests.get(STATS_URL)
resp = req.json()

top_langs = filter(
    lambda n: n["name"] not in EXCLUDED_LANGS,
    resp['data']['languages']
)

# Export compiled readme
with open("./readme.md", "w+", encoding="utf-8") as readme_out:
    readme_out.write(
        template.render(
            full = resp['data'],
            top_langs = top_langs,
            ascii_progress_bar=ascii_progress_bar
        )
    )
