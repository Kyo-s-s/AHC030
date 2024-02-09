import os

script_directory = os.path.dirname(os.path.realpath(__file__))
src_path = os.path.join(script_directory, 'main/src')
main_path = os.path.join(src_path, 'main.rs')

content = ""

def insert(path):
    global content
    with open(path) as file:
        bandle = False
        for line in file:
            if bandle:
                if line.startswith("// --- bandle off ---"):
                    bandle = False
                if line.startswith("// path:"):
                    path = line.split(":")[1].strip()
                    content += "\n" + line
                    insert(os.path.join(src_path, path))
            else:
                if line.startswith("// --- bandle on ---"):
                    bandle = True
                else:
                    content += line

insert(main_path)

output_file_path = os.path.join(script_directory, 'bandle/src/main.rs')
with open(output_file_path, 'w') as fout:
    fout.write(content)
