# This Python script will enumerate all files inside of the "resource" folder,
# and create assoctaited HTTP resources for Proxide by saving these
# associations to "proxide_manifest.csv". This CSV is loaded by Proxide at
# startup, and converted into a Hashmap for quick indexing.
#
# This approach is intentionally limited, in that it does not permit
# file uploading of any kind. While this may have been implemented, it was
# not a part of our original plan, and we hadn't the time to consider it.
#
# @author Matthew Kenneth Peterson
# @date April 3, 2024

import os

def list_all_files(ROOTDIR):
	"""
	This function iterates through a directory and its subdirectories,
	yielding the full paths of all existing files.

	Args:
		ROOTDIR: The starting directory path.
  	"""
	for root, directories, files in os.walk(ROOTDIR):
		for filename in files:
	  		# Construct the full path of the file
	  		full_path = os.path.join(root, filename)
	  
  		# Check if the file exists before yielding
		if os.path.isfile(full_path):
			yield full_path


if __name__ in "__main__":
	file = open("./proxide_manifest.csv", "w")
	file.write("Resource,Request\n")
	for file_path in list_all_files(ROOTDIR):
		file.write(str("/" + file_path.split(ROOTDIR)[1] + "," + file_path[1:].replace('\\', '/') + "\n"))

	# Add custom redirects
	CUSTOM = [ '/,./resources/home.html\n', '/home,./resources/home.html\n' ]

	for c in CUSTOM:
		file.write(c)
