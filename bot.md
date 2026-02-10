# Documentation about API calls for the bot

## curl for OpenRouter AI request

```bash
curl https://openrouter.ai/api/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $OPENROUTER_KEY" \
  -d '{
  "model": "z-ai/glm-4.7",
  "messages": [
    {
      "role": "user",
      "content": "Tell me a short story about the history of New York."
    }
  ],
  "reasoning": {
    "enabled": true
  }
}'
```

This is the json response

```json
{
  "id": "gen-1770718795-pLSc83lE5pAPjuM3IcdO",
  "provider": "Nebius",
  "model": "z-ai/glm-4.7",
  "object": "chat.completion",
  "created": 1770718795,
  "choices": [
    {
      "logprobs": null,
      "finish_reason": "stop",
      "native_finish_reason": "stop",
      "index": 0,
      "message": {
        "role": "assistant",
        "content": "Longbefore the skyline pierced the clouds, the island was a forested hump of granite rising out of the harbor. The Lenape people called it *Mannahatta*, meaning \"hilly island.\" They hunted deer in the woods and fished the teeming waters in birchbark canoes, unaware that their home would one day become the center of the world.\n\nThe transformation began in 1624, not with a bang, but with a business transaction. The Dutch West India Company sent settlers to establish a fur-trading outpost. They bought the island from the Lenape for goods worth 60 guilders—legendarily, the equivalent of $24—and named it New Amsterdam. It was a chaotic, muddy collection of taverns and warehouses. To protect their northern border, the Dutch built a wall across the island. Today, we know that path as Wall Street.\n\nIn 1664, the British sailed into the harbor and demanded the city surrender. The Dutch governor, Peter Stuyvesant, was furious, but the people wanted peace. The Union Jack went up, and New Amsterdam became New York, named after the Duke of York.\n\nFor the next century, the city grew slowly, serving as a strategic port during the Revolutionary War. It was occupied by British troops, burned, and rebuilt. But its true explosion came in the 19th century with the opening of the Erie Canal in 1825. This man-made waterway turned New York into the gateway to the American interior, connecting the Atlantic to the Great Lakes. The city became the Empire City, pumping wealth and goods through its veins.\n\nThen came the people. In 1892, Ellis Island opened its doors. Millions arrived—Irish, Italians, Germans, Jews, and Poles—fleeing famine, war, and poverty, seeking a fresh start. They crowded into tenements on the Lower East Side, building the skyscrapers of the city's soul with their labor. The skyline began to rise, a steel forest built on bedrock, powered by the invention of the elevator and the ambition of men like J.P. Morgan and John D. Rockefeller.\n\nThe 20th century saw New York become the capital of culture and finance. The Jazz Age roared through Harlem and Midtown, and by the time the Empire State Building was completed in 1931, the city was a literal beacon of possibility, the \"City that Never Sleeps.\"\n\nBut New York has never been without its scars. It faced financial ruin in the 1970s, only to bounce back. It faced the terror of September 11, 2001, when the Twin Towers fell, plunging the city into silence. Yet, true to its history, New York rebuilt itself, resilient and defiant.\n\nToday, standing on a street corner in Manhattan, you can see the history in the layers of brick and steel: a Dutch colonial church tucked between office towers, a brownstone from the Gilded Age, a glass spire reflecting the sun. It is a city born from a forest, bought for trinkets, built by immigrants, and defined by an endless, relentless drive to become something new.",
        "refusal": null,
        "reasoning": "1.  **Analyze the Request:**\n    *   **Topic:** The history of New York.\n    *   **Format:** A short story.\n    *   **Goal:** Tell the history through a narrative lens rather than just a list of facts.\n\n2.  **Determine the Scope and Structure:**\n    *   New York's history is vast (from Lenape land to modern metropolis).\n    *   To make it a \"story,\" I need a narrative arc. I can use a personified character (the city itself) or a \"fly on the wall\" perspective, or simply a chronological narrative with evocative language.\n    *   *Decision:* I will use a chronological narrative style, but focused on the *feeling* of the city evolving. I'll break it down into key eras:\n        *   Era 1: The Lenape (The beginning).\n        *   Era 2: New Amsterdam (The Dutch trading post).\n        *   Era 3: New York (The British takeover and growth).\n        *   Era 4: The Immigration Boom (Ellis Island, the melting pot).\n        *   Era 5: The Rise of the Skyscraper (The modern city).\n        *   Era 6: Resilience/Modern Day (9/11, comeback).\n\n3.  **Drafting - Section by Section:**\n\n    *   *Introduction (The Land):* Start before the concrete. The island of Mannahatta. Forests, hills, the Lenape people walking the paths. It’s quiet, wild.\n    *   *The Arrival of Ships (The Dutch):* 1624. The *Halve Maen*. The wall (Wall Street) built for defense. The purchase of Manhattan (legendarily for $24, though actually trinkets). The messy, muddy port town of New Amsterdam. A place of commerce.\n    *   *The Shift (The British):* 1664. The fleet arrives. Peter Stuyvesant surrenders. It becomes New York. It grows, but it’s still chaotic. The fire of 1776 (Revolutionary War context briefly). It becomes the capital briefly, then grows on trade.\n    *   *The Great Tides (Immigration):* Late 1800s/Early 1900s. This is crucial. The smell of the harbor, the noise of Ellis Island. The Statue of Liberty as the beacon. Tenements, crowded streets, the clash of languages. The city becomes a \"mosaic.\"\n    *   *Reaching for the Sky (The Skyscrapers):* The bedrock of Manhattan allows for this. The Woolworth Building, the Empire State. The city creates a \"canyon\" of steel. The Jazz Age, the energy. It’s the center of the world.\n    *   *Hardship and Resilience (Modern Era):* The 1970s (grime and crime, briefly mentioned) or the 9/11 attacks. I'll focus on 9/11 as the pivotal modern trauma and rebirth. The hole in the ground, then the Freedom Tower.\n    *   *Conclusion:* Summarize the spirit. It’s a city that never sleeps, built on dreams and grit.\n\n4.  **Refining and Polishing (Self-Correction during drafting):**\n    *   *Critique:* Is this too textbook?\n    *   *Fix:* Use sensory details. Don't say \"The Dutch built a wall.\" Say \"They pounded wooden stakes into the mud on a path that would one day be the world's financial center.\"\n    *   *Critique:* Is it too long?\n    *   *Fix:* Keep descriptions concise. Focus on the *mood* of the changes.\n\n5.  **Drafting the Narrative (Mental Simulation):**\n\n    *   *Opening:* \"Once, the island was just a hump of granite and green...\" Mannahatta.\n    *   *The Dutch:* Then the sails appeared. Beavers. The wall.\n    *   *The British:* The name changed, but the greed for trade remained.\n    *   *The Immigrants:* \"The Golden Door.\" The teeming streets.\n    *   *The Skyscrapers:* \"They defied gravity.\"\n    *   *Ending:* \"It is a story of constant becoming.\"\n\n6.  **Final Polish (similar to the generated output):**\n    *   Ensure the transition from the Lenape to the Dutch is respectful of the indigenous history.\n    *   Make sure the \"Wall Street\" connection is clear.\n    *   End with a hopeful note about the city's endurance.\n\n7.  **Final Output Generation** (This matches the provided good response).",
        "reasoning_details": [
          {
            "format": "unknown",
            "index": 0,
            "type": "reasoning.text",
            "text": "1.  **Analyze the Request:**\n    *   **Topic:** The history of New York.\n    *   **Format:** A short story.\n    *   **Goal:** Tell the history through a narrative lens rather than just a list of facts.\n\n2.  **Determine the Scope and Structure:**\n    *   New York's history is vast (from Lenape land to modern metropolis).\n    *   To make it a \"story,\" I need a narrative arc. I can use a personified character (the city itself) or a \"fly on the wall\" perspective, or simply a chronological narrative with evocative language.\n    *   *Decision:* I will use a chronological narrative style, but focused on the *feeling* of the city evolving. I'll break it down into key eras:\n        *   Era 1: The Lenape (The beginning).\n        *   Era 2: New Amsterdam (The Dutch trading post).\n        *   Era 3: New York (The British takeover and growth).\n        *   Era 4: The Immigration Boom (Ellis Island, the melting pot).\n        *   Era 5: The Rise of the Skyscraper (The modern city).\n        *   Era 6: Resilience/Modern Day (9/11, comeback).\n\n3.  **Drafting - Section by Section:**\n\n    *   *Introduction (The Land):* Start before the concrete. The island of Mannahatta. Forests, hills, the Lenape people walking the paths. It’s quiet, wild.\n    *   *The Arrival of Ships (The Dutch):* 1624. The *Halve Maen*. The wall (Wall Street) built for defense. The purchase of Manhattan (legendarily for $24, though actually trinkets). The messy, muddy port town of New Amsterdam. A place of commerce.\n    *   *The Shift (The British):* 1664. The fleet arrives. Peter Stuyvesant surrenders. It becomes New York. It grows, but it’s still chaotic. The fire of 1776 (Revolutionary War context briefly). It becomes the capital briefly, then grows on trade.\n    *   *The Great Tides (Immigration):* Late 1800s/Early 1900s. This is crucial. The smell of the harbor, the noise of Ellis Island. The Statue of Liberty as the beacon. Tenements, crowded streets, the clash of languages. The city becomes a \"mosaic.\"\n    *   *Reaching for the Sky (The Skyscrapers):* The bedrock of Manhattan allows for this. The Woolworth Building, the Empire State. The city creates a \"canyon\" of steel. The Jazz Age, the energy. It’s the center of the world.\n    *   *Hardship and Resilience (Modern Era):* The 1970s (grime and crime, briefly mentioned) or the 9/11 attacks. I'll focus on 9/11 as the pivotal modern trauma and rebirth. The hole in the ground, then the Freedom Tower.\n    *   *Conclusion:* Summarize the spirit. It’s a city that never sleeps, built on dreams and grit.\n\n4.  **Refining and Polishing (Self-Correction during drafting):**\n    *   *Critique:* Is this too textbook?\n    *   *Fix:* Use sensory details. Don't say \"The Dutch built a wall.\" Say \"They pounded wooden stakes into the mud on a path that would one day be the world's financial center.\"\n    *   *Critique:* Is it too long?\n    *   *Fix:* Keep descriptions concise. Focus on the *mood* of the changes.\n\n5.  **Drafting the Narrative (Mental Simulation):**\n\n    *   *Opening:* \"Once, the island was just a hump of granite and green...\" Mannahatta.\n    *   *The Dutch:* Then the sails appeared. Beavers. The wall.\n    *   *The British:* The name changed, but the greed for trade remained.\n    *   *The Immigrants:* \"The Golden Door.\" The teeming streets.\n    *   *The Skyscrapers:* \"They defied gravity.\"\n    *   *Ending:* \"It is a story of constant becoming.\"\n\n6.  **Final Polish (similar to the generated output):**\n    *   Ensure the transition from the Lenape to the Dutch is respectful of the indigenous history.\n    *   Make sure the \"Wall Street\" connection is clear.\n    *   End with a hopeful note about the city's endurance.\n\n7.  **Final Output Generation** (This matches the provided good response)."
          }
        ]
      }
    }
  ],
  "usage": {
    "prompt_tokens": 17,
    "completion_tokens": 1640,
    "total_tokens": 1657,
    "cost": 0.0032868,
    "is_byok": false,
    "prompt_tokens_details": {
      "cached_tokens": 0,
      "audio_tokens": 0
    },
    "cost_details": {
      "upstream_inference_cost": 0.0032868,
      "upstream_inference_prompt_cost": 0.0000068,
      "upstream_inference_completions_cost": 0.00328
    },
    "completion_tokens_details": {
      "reasoning_tokens": 977,
      "audio_tokens": 0
    }
  }
}
```

