#!/bin/bash
pod_name_prefix="ore"
for i in {1..3}; do
	cp ./o.yaml $pod_name_prefix$i.yaml
	export name=$pod_name_prefix$i
	/usr/bin/yq -i '.metadata.name = strenv(name)' $pod_name_prefix$i.yaml
	kubectl apply -f ./$pod_name_prefix$i.yaml
	# kubectl delete -f ./c$i.yaml
done
