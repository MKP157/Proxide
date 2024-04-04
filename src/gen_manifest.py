import os

def list_all_files(root_dir):
  """
  This function iterates through a directory and its subdirectories,
  yielding the full paths of all existing files.

  Args:
    root_dir: The starting directory path.
  """
  for root, directories, files in os.walk(root_dir):
    for filename in files:
      # Construct the full path of the file
      full_path = os.path.join(root, filename)
      # Check if the file exists before yielding
      if os.path.isfile(full_path):
        yield full_path

# Example usage
root_dir = "../resources/"  # Replace with your actual directory path

file = open("./proxide_manifest.csv", "w")
file.write("Resource,Request\n")
for file_path in list_all_files(root_dir):
  file.write(str("/" + file_path.split(root_dir)[1] + "," + file_path[1:].replace('\\', '/') + "\n"))

# Add custom redirects
CUSTOM = [ '/,./resources/home.html\n', '/home,./resources/home.html\n' ]

for c in CUSTOM:
  file.write(c)
