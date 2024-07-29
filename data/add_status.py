import os
import json

# List of ongoing shortcodes
ongoing_shortcodes = [
		"0102",
		"0103",
		"0105",
		"0106",
		"0107",
		"0112",
		"0114",
		"0116",
		"0118",
		"0119",
		"0121",
		"0801",
		"0804",
		"0805",
		"0806",
		"0807",
		"080C",
		"080E",
		"0810",
		"0812",
		"0813",
		"0816",
		"081B",
		"0820",
		"0827",
		"082B",
		"082C",
		"0836",
		"0838",
		"083A",
		"083B",
		"083C",
		"083E",
		"0843",
		"0844",
		"0849",
		"084A",
		"084E"
]

def update_json_files(folder_path):
    print(ongoing_shortcodes)
    # Iterate through all files in the given folder
    for filename in os.listdir(folder_path):
        # Check if the file is a JSON file
        if filename.endswith(".json"):
            file_path = os.path.join(folder_path, filename)

            # Read the JSON file
            with open(file_path, 'r', encoding='utf-8') as file:
                data = json.load(file)

            # Check if the "project" and "shortcode" keys exist
            if "project" in data and "shortcode" in data["project"]:
                shortcode = data["project"]["shortcode"]

                # Determine the status based on the shortcode
                print (shortcode)
                print (shortcode in ongoing_shortcodes)
                if shortcode in ongoing_shortcodes:
                    status = "ongoing"
                else:
                    status = "finished"

                # Add or update the "status" property
                data["project"]["status"] = status

                # Write the updated JSON back to the file
                with open(file_path, 'w', encoding='utf-8') as file:
                    json.dump(data, file, indent=4, ensure_ascii=False)
            else:
                print(f"File {filename} does not contain the expected structure.")

update_json_files('./json/')