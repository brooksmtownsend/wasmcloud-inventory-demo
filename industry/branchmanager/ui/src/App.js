import React, { useState, useEffect } from 'react';
import {
  ChakraProvider,
  Box,
  Text,
  VStack,
  Grid,
  Input,
  theme,
  Button,
  Menu, MenuButton, MenuList, MenuItem,
  Table, Thead, Tbody, Tr, Th, Td, HStack
} from '@chakra-ui/react';
import { ColorModeSwitcher } from './ColorModeSwitcher';
import { Logo } from './Logo';

const WASMCLOUD_COLOR = "#00C389";
const NEW_ITEM_ID = "new_item_placeholder_id";

function App() {
  const [inventoryData, setInventoryData] = useState([]);

  const fetchInventory = () => {
    // Fetch data from your API endpoint
    fetch("/inventory")
      .then((response) => response.json())
      .then((data) => setInventoryData(data.flat(Infinity).sort((a, b) => a.item_type.localeCompare(b.item_type))))
      .catch((error) => {
        console.error("Error fetching data:", error);
      });
  };

  const [selectedItem, setSelectedItem] = React.useState(null);

  const handleMenuItemClick = (itemValue) => {
    setSelectedItem(itemValue);
  };

  const newOrder = () => {
    fetch("/order", { method: "POST", body: JSON.stringify(selectedItem) })
      .then((response) => console.dir(response))
      .then((_response) => fetchInventory())
      .catch((error) => {
        console.error("Error setting name:", error);
      });
  }

  const newShipment = () => {
    fetch("/shipment", { method: "POST", body: JSON.stringify(selectedItem) })
      .then((response) => console.dir(response))
      .then((_response) => fetchInventory())
      .catch((error) => {
        console.error("Error setting name:", error);
      });
  }

  const setName = () => {
    fetch("/name", { method: "POST", body: branchName })
      .then((response) => console.dir(response))
      .catch((error) => {
        console.error("Error setting name:", error);
      });
  }

  const [branchName, setBranchName] = React.useState("");

  const fetchName = () => {
    fetch("/name")
      .then((response) => {
        if (response.ok) {
          return response.text()
        } else {
          return ""
        }
      })
      .then((name) => setBranchName(name))
      .catch((error) => {
        console.error("Error fetching name:", error);
      });
  };

  const handleSetBranchName = (itemValue) => {
    setBranchName(itemValue.nativeEvent.target.value);
  };

  useEffect(() => {
    // Fetch data when the component mounts
    fetchName();
    fetchInventory();
  }, []);

  return (
    <ChakraProvider theme={theme}>
      <Box textAlign="center" fontSize="xl">
        <Grid p={3}>
          <ColorModeSwitcher justifySelf="flex-end" />
          <VStack spacing={4}>
            <Logo h="200px" pointerEvents="none" />
            <Box width="100%" maxWidth="75vw">
              <Box textAlign="center" mb={2}>
                <Text fontSize="2xl">
                  Unit Inventory
                </Text>
                <HStack justifyContent="center" overflowWrap overflowX="auto">
                  <Input
                    width="auto"
                    placeholder="Enter your branch name"
                    value={branchName}
                    onChange={handleSetBranchName}
                  />
                  <Button color={WASMCLOUD_COLOR} onClick={setName}>
                    Set Name
                  </Button>
                </HStack>
                <Menu>
                  <MenuButton as={Button} rightIcon="▼" mt={2}>
                    New Shipment or Order
                  </MenuButton>
                  <MenuList>
                    <MenuItem key={"new"} onClick={() => setSelectedItem({ id: NEW_ITEM_ID, quantity: 0, item_type: "" })}>New Item</MenuItem>
                    {inventoryData.map((item) => (
                      <MenuItem key={item.id} onClick={() => handleMenuItemClick(item)}>{item.item_type}</MenuItem>
                    ))}
                    <MenuItem key={"new"} onClick={() => setSelectedItem(null)}>Cancel</MenuItem>
                  </MenuList>
                </Menu>
                <HStack justifyContent="center" mt={2} mb={2}>
                  {selectedItem && <>
                    <Input width="auto" value={selectedItem ? selectedItem.item_type : ""} placeholder="Item Type" onChange={(e) => {
                      setSelectedItem({ ...selectedItem, item_type: e.nativeEvent.target.value })
                    }} disabled={selectedItem.id !== NEW_ITEM_ID} />
                    <Input width="auto" value={selectedItem ? selectedItem.quantity : 0} onChange={(e) => {
                      const newQuantity = Number.parseInt(e.nativeEvent.target.value);
                      setSelectedItem({ ...selectedItem, quantity: isNaN(newQuantity) ? 0 : newQuantity })
                    }} />
                    <Button minWidth="150px" color={WASMCLOUD_COLOR} onClick={newShipment}> New Shipment </Button>
                    {selectedItem && selectedItem.id !== NEW_ITEM_ID && <Button color="red" onClick={newOrder}> New Order </Button>}
                  </>}
                </HStack>
                <hr />
                <Table variant="simple" width="100%" maxWidth="50vw" mx="auto">
                  <Thead>
                    <Tr>
                      <Th>Item</Th>
                      <Th textAlign="right">Quantity</Th> {/* Right-align Quantity column */}
                    </Tr>
                  </Thead>
                  <Tbody>
                    {inventoryData.map((item) => (
                      <Tr key={item.id}>
                        <Td>{item.item_type}</Td>
                        <Td textAlign="right">{item.quantity}</Td>
                      </Tr>
                    ))}
                  </Tbody>
                </Table>
              </Box>
            </Box>
          </VStack>
        </Grid>
      </Box>
    </ChakraProvider >
  );
}

export default App;
