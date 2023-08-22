import torch
import torch.nn as nn
import numpy as np
from numpy import genfromtxt
from snntorch import spikegen
from mnist import loadDataset
from runParameters import *

data_path='./data/mnist'
outputFilename = "output.txt"
targetsFile = "targets.txt"

train_loader, test_loader = loadDataset(data_path, batch_size)

# Prendo solo il primo batch di 256 immagini
test_data, test_targets = next(iter(test_loader))

# Creo batch di 256 input spikes (256 x 784 x 100)
input_spikes = spikegen.rate(test_data.view(batch_size, -1), num_steps = num_steps, gain = 1)
new_data = np.transpose(input_spikes.numpy(),(1,0,2))

# Li scrivo su un file
data = new_data
with open('inputSpikes.txt', 'w') as outfile:
	# I'm writing a header here just for the sake of readability
    	outfile.write('# Array shape: {0}\n'.format(data.shape))
	# Iterating through a ndimensional array produces slices along
    	# the last axis. This is equivalent to data[i,:,:] in this case
    	for data_slice in data:
        	np.savetxt(outfile, data_slice, fmt='%d')
		# Writing out a break to indicate different slices
        	outfile.write('# New slice\n')

with open(targetsFile, 'w') as outfile:
	np.savetxt(outfile, test_targets, fmt='%d')

# Lancio il programma rust con i parametri e questi input 
import subprocess as sp
rustScript = "../target/debug/main"
sp.run(rustScript)

# Alla fine l'output è di dimensione 100 x 256 x 10
# Lo scrivo su un file e lo salvo come tensor
a = genfromtxt(outputFilename, delimiter=',')
my_data_t = torch.from_numpy(a)
print(test_targets)
print(my_data_t)

# Alla fine calcolo l'accuratezza con queste formule
acc = np.mean((test_targets == my_data_t).detach().cpu().numpy())
print(f"Test set accuracy for a single minibatch: {acc*100:.2f}%")
