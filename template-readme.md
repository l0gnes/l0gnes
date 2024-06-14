# 👋 Hello World, I'm Alex!

- 🪴 Backend Developer @Voithos-Development
- 🐍 Proficient in Python

## 💚 My Top Languages
```{% for lang in top_langs[:5] %}
{{ loop.index }}. {{lang['name']}}   {{ ascii_progress_bar(lang['percent'] / 100).rjust(40 - lang['name']|length) }} {{ lang['text']}}{% endfor %}
```
- 💪 Daily Average: `{{ full["human_readable_daily_average"] }}`
- 🕑 All Time: `{{ full["human_readable_total_including_other_language"] }}`
